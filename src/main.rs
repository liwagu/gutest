enum Kind {
    Build,
    Test,
    Doc,
    // ... 其他种类
}

#[derive(Debug)]
struct StepDescription {
    name: String,
}

// 简化的 Builder 结构体
struct Builder;

impl Builder {
        // 这是一个关联函数，不是方法
    fn get_step_descriptions(kind: Kind) -> Vec<StepDescription> {
        
        // 定义了一个名为 describe 的宏，
        // 用于生成一个包含 StepDescription 结构体实例的向量。
        macro_rules! describe {
            ($($step:expr),+ $(,)?) => {{ //匹配表达式，逗号可选
                vec![$(StepDescription { name: $step.to_string() }),+]
                //宏展开：生成一个包含 StepDescription 结构体实例的向量。
            }};
        }

        // 根据不同的 Kind 返回不同的步骤描述
        match kind {
            Kind::Build => describe!("Compile", "Link", "Package"),
            Kind::Test => describe!("UnitTest", "IntegrationTest"),
            Kind::Doc => describe!("GenerateDocs", "PublishDocs"),
            // ... 其他匹配分支
        }
    }
}

fn main() {
    // 使用 :: 语法调用关联函数
    // 获取 Build 类型的步骤描述
    let build_steps = Builder::get_step_descriptions(Kind::Build);
    println!("构建Build steps: {:?}", build_steps);

    // 获取 Test 类型的步骤描述
    let test_steps = Builder::get_step_descriptions(Kind::Test);
    println!("测试Test steps: {:?}", test_steps);
}