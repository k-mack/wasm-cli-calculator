use std::error::Error;
use std::fmt;
use wasmtime::*;

type Numeric = f64;

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse CLI args
    let mut args = std::env::args();

    let Some(app_name) = args.next() else {
        panic!("Error: program name not available");
    };

    let Some(Ok(x)) = args.next().map(|s| s.parse::<Numeric>()) else {
        usage(&app_name);
        return Ok(());
    };
    let Some(Ok(y)) = args.next().map(|s| s.parse::<Numeric>()) else {
        usage(&app_name);
        return Ok(());
    };
    let Some(op) = args.next() else {
        usage(&app_name);
        return Ok(());
    };

    // Create wasmtime engine with the Wasm component model enabled
    let engine = Engine::new(Config::new().wasm_component_model(true))?;

    // Load and compile the Wasm module
    let component: component::Component =
        component::Component::from_file(&engine, "calculator-composed.wasm")
            .expect("calculator-composed.wasm not found");

    // Create a `Store` to own all Wasm-related things (instances, functions, etc.)
    let mut store = Store::new(&engine, ());

    // Create a linker to instantiate components
    let mut linker = component::Linker::new(&engine);

    // Get the component's type
    let component_type = linker
        .substituted_component_type(&component)
        .expect("could not get component type from linker");

    // We know the component (i.e., the world) we want to use is `calc:basic/calculate@0.1.0`, so we need to query the Component struct for this.
    let component_item = component_type
        .get_export("calc:basic/calculate@0.1.0")
        .expect("calculate component not found");
    let instance_type;
    if let component::types::ComponentItem::ComponentInstance(instance) = component_item {
        instance_type = instance;
    } else {
        return Err(Box::new(MyError(
            "expected calculate to be a component instance".to_string(),
        )));
    }

    // We have the component instance we want. This instance should also export the enum representing the calculator's support operations. We need to query for that now.
    let op_val;
    if let component::types::ComponentItem::Type(component::Type::Enum(ty)) = instance_type
        .get_export("op")
        .expect("op enum not found in component instance")
    {
        op_val = ty
            .new_val(&op)
            .expect(format!("unsupported operation \"{}\"", op).as_str());
    } else {
        return Err(Box::new(MyError(
            "expected op enum in calculate component instance".to_string(),
        )));
    }

    // Finally, we need to query the component for the `eval-expression` function.
    let func_type;
    if let component::types::ComponentItem::ComponentFunc(ty) = instance_type
        .get_export("eval-expression")
        .expect("eval-expression function not found in component instance")
    {
        func_type = ty;
    } else {
        return Err(Box::new(MyError(
            "expected eval-expression function in calculate component instance".to_string(),
        )));
    }

    // Get the component's exports
    let exports = component_type.exports();

    // Print exports
    let mut op_enum_type = None;
    for (name, item) in exports {
        println!("export found: {{name='{}', item={:?}'}}", name, item);
        match item {
            component::types::ComponentItem::ComponentInstance(instance) => {
                println!("  component instance: {:?}", instance);
                for (name, item) in instance.exports() {
                    println!("    export found: {:?}", (name, &item));
                    match item {
                        component::types::ComponentItem::Type(ty) => match ty {
                            component::Type::Enum(e) => {
                                println!("      enum found: {:?}", e);
                                if name == "op" {
                                    op_enum_type = Some(e);
                                }
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    println!("did I find the enum: {:?}", &op_enum_type);
    println!(
        "cool. now create the requested variant: {:?}",
        op_enum_type.as_ref().unwrap().new_val(&op).unwrap()
    );
    println!(
        "sweet. now can I get enum with a direct lookup call: {:?}",
        component_type.get_export("calc:basic/calculate@0.1.0")
    );

    // Instantiate the component (b/c a component is just a code representation)
    let instance: component::Instance = linker.instantiate(&mut store, &component)?;

    // Look up `op` enum type
    //let mut exports: component::Exports = instance.exports(&mut store);
    //let op_enum_instance: component::ExportInstance = exports.instance("op").unwrap();//.ok_or(Err(Box::new(MyError("cannot get `op` enum export instance".into())))).unwrap();

    //instance.exports(&mut store).root().modules().for_each(|(str, module)| println!("exported module: {} or {}", str, module.name().unwrap()));

    // Looks up the exported function `eval_expression`
    //let eval_expression_func = instance
    //    .get_func(&mut store, "eval-expression")
    //    .expect("`eval_expression` is not an exported function");

    //// Instantiate the enum Op code type defined within the component
    //let calc_op_type = instance.get_export("op")?.unwrap_enum()?; //component::types::Enum::new_val("op")?;

    //// Instantiate the specified Op code enum case
    //let calc_op_case = component::Enum::new(&calc_op_type, op.to_lowercase().as_str());

    // Invoke the exported function based on the application's input
    let params = [
        op_val,
        component::Val::Float64(x),
        component::Val::Float64(y),
    ];
    let mut result = [component::Val::Float64(0.0)];
    let op_symbol = match op.to_lowercase().as_str() {
        "add" => "+",
        "subtract" => "-",
        _ => "?",
    };

    // Let's put everything we learned together:
    // 1. The component loaded from file is just a unit of code.
    // 2. This particular component, through its wit "world" definition, exports the interface `calc:basic/calculate@0.1.0`.
    // 3. Within this exported interface, there is the function `eval-expression` that we want to invoke.
    // 4. Therefore, we need to instantiate the component, query its exports for the interface containing the function we want to invoke, and then invoke the function with the desired parameters.
    let func = instance
        .exports(&mut store)
        .instance("calc:basic/calculate@0.1.0")
        .expect("expected exported interface \"calc:basic/calculate@0.1.0\" not found")
        .func("eval-expression")
        .expect("expected exported function \"eval-expression\" not found");
    func.call(&mut store, &params, &mut result)?;

    // The below doesn't work b/c the function isn't defined at the top level of the component
    //let _ = instance.get_func(&mut store, "eval-expression").expect("eval-expression not found in component instantiation").call(&mut store, &params, &mut result)?;

    println!(
        "{} {} {} = {:?}",
        x,
        op_symbol,
        y,
        if let component::Val::Float64(v) = result[0] {
            v
        } else {
            return Err(Box::new(MyError(
                "Calculator did not return a float64".to_string(),
            )));
        }
    );

    Ok(())
}

fn usage(app_name: &str) {
    println!("usage: {} <number> <number> <add | subtract>", app_name);
}
