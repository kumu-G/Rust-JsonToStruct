use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json = r#"
    {
        "article": "how to work with json in rust",
        "author": "qiao",
        "paragraph": [
            { "name": "starting sentence" },
            { "name": "body of the paragraph" },
            { "name": "end of the paragraph" }
        ]
    }"#;

    let parsed: Article = read_json_typed(json);

    println!(
        "\n\n The name of the first paragraph is : {}",
        parsed.paragraph[0].name
    );
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}

/*
这段代码的功能是将一个 JSON 字符串解析为一个 Article 结构体，并打印出第一个段落的名称。

代码的步骤如下：

1. 导入 serde 库中的 Deserialize 和 Serialize trait。
2. 定义了一个名为 Paragraph 的结构体，其中包含一个名为 name 的字符串字段。
3. 定义了一个名为 Article 的结构体，其中包含一个名为 article 的字符串字段，一个名为 author 的字符串字段，以及一个名为 paragraph 的 Paragraph 结构体的向量字段。
4. 在 main 函数中，定义了一个 JSON 字符串，并赋值给变量 json。
5. 调用 read_json_typed 函数，并将 json 作为参数传递进去，将返回值赋值给变量 parsed。
6. 使用 println! 宏打印出第一个段落的名称，即 parsed.paragraph[0].name。
7. 定义了一个名为 read_json_typed 的函数，接受一个名为 raw_json 的字符串参数，并返回一个 Article 结构体。
8. 在 read_json_typed 函数中，使用 serde_json 库的 from_str 函数将 raw_json 解析为一个 Article 结构体，并将结果赋值给变量 parsed。
9. 返回 parsed。

代码的作用是将给定的 JSON 字符串解析为一个 Article 结构体，并打印出第一个段落的名称。 */
