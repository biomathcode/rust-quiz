use std::env;
use std::fs;
use inquire::Select;

#[derive(Debug, Clone, PartialEq)]
pub struct Data {
    question: String,
    options: Vec<String>,
    right_option: String,
}

fn parser(content: String) -> Result<Vec<Data>, &'static str> {
    let mut datanew: Vec<Data> = Vec::new();

    let contentlist: Vec<&str> = content.split('\n').to_owned().collect();

    let mut prevquestion = String::new();

    let options_vec: Vec<String> = Vec::new();

    let mut right_ = String::new();

    for i in contentlist {
        let question_match = i.starts_with("--");

        let option_match = i.starts_with("- [ ]");

        let right_option = i.starts_with("- [x]");

        if question_match {
            prevquestion = i.to_owned();
            datanew.push(Data {
                question: prevquestion.clone(),
                options: options_vec.clone(),
                right_option: right_.clone(),
            });
        }

        if option_match || right_option {
            let already_exist = datanew
                .iter()
                .position(|x| { x.question == prevquestion })
                .unwrap();

            let mut old_vec = datanew
                .iter()
                .find(|x| x.question == prevquestion)
                .unwrap()
                .options.clone();

            old_vec.push(i[5..].to_string());

            if right_option {
                right_ = i[5..].to_string();
            }

            let newvalue = Data {
                question: prevquestion.clone(),
                options: old_vec.to_vec(),
                right_option: right_.clone(),
            };

            datanew[already_exist] = newvalue;
        }

        // map.insert(k, v);
    }
    Ok(datanew)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let datanew = parser(contents).unwrap();

    let total_question = datanew.len();

    let mut correct_question = 0;

    let mut wrong_question = 0;

    for i in datanew {
        let ans = Select::new(&i.question.to_string(), i.options).prompt();

        match ans {
            Ok(choice) => {
                if choice == i.right_option {
                    correct_question += 1;
                    println!("Correct Answer: {}", choice);
                } else {
                    wrong_question += 1;
                    println!("Wrong Answer: {}", choice);
                }
            }
            Err(_) => println!("There was an error, please try again"),
        }
    }

    println!(
        "\n \nTotal Questions: {} \nCorrect Answers: {} \nWrong Answers {}\n\n ",
        total_question,
        correct_question,
        wrong_question
    )

    // println!("With text:\n{contents}");
}
