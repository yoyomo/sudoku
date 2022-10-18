struct Answer {
    row: usize,
    column: usize,
    number: usize,
}

fn init_possible_answers(answers: &mut Vec<Vec<u8>>) -> Vec<Vec<Vec<u8>>> {
    let mut possible_answers: Vec<Vec<Vec<u8>>> = vec![vec![vec![1; 9]; 9]; 9];

    for r in 0..9 {
        for c in 0..9 {
            let square = answers[r][c];
            if square > 0 {
                update_answer(
                    answers,
                    &mut possible_answers,
                    &Answer {
                        number: (square - 1) as usize,
                        row: r,
                        column: c,
                    },
                )
            }
        }
    }

    return possible_answers;
}

fn update_answer(
    answers: &mut Vec<Vec<u8>>,
    possible_answers: &mut Vec<Vec<Vec<u8>>>,
    answer: &Answer,
) {
    let row = answer.row;
    let column = answer.column;
    let number = answer.number;

    answers[row][column] = (number + 1) as u8;
    possible_answers[row][column] = vec![0; 9];

    for c in 0..9 {
        possible_answers[row][c][number] = 0;
    }

    for r in 0..9 {
        possible_answers[r][column][number] = 0;
    }

    for r in 0..3 {
        for c in 0..3 {
            possible_answers[(row / 3) * 3 + r][(column / 3) * 3 + c][number] = 0;
        }
    }
}

fn found_all_answers(answers: &Vec<Vec<u8>>) -> bool {
    for row in 0..9 {
        for column in 0..9 {
            if answers[row][column] == 0 {
                return false;
            }
        }
    }

    return true;
}

fn find_one_answer(answers: &mut Vec<Vec<u8>>, possible_answers: &mut Vec<Vec<Vec<u8>>>) -> bool {
    let mut found_at_least_one_answer = false;
    let mut least_numbered_answers = Vec::<Answer>::new();
    let mut least_number_count = usize::max_value();
    for row in 0..9 {
        for column in 0..9 {
            let mut linear_answers = Vec::<Answer>::new();
            for answer in 0..9 {
                if possible_answers[row][column][answer] == 1 {
                    linear_answers.push(Answer {
                        number: answer,
                        row: row,
                        column: column,
                    });
                }
            }

            if linear_answers.len() == 1 {
                update_answer(answers, possible_answers, &linear_answers[0]);
                found_at_least_one_answer = true;
            } else if linear_answers.len() > 1 && linear_answers.len() < least_number_count {
                least_number_count = linear_answers.len();
                least_numbered_answers = linear_answers;
            }
        }
    }

    if !found_at_least_one_answer && least_numbered_answers.len() > 1 {
        for answer in least_numbered_answers {
            let mut alternate_answers = answers.to_vec();
            let mut alternate_possible_answers = possible_answers.to_vec();

            update_answer(
                &mut alternate_answers,
                &mut alternate_possible_answers,
                &answer,
            );

            while find_one_answer(&mut alternate_answers, &mut alternate_possible_answers) {}
            if found_all_answers(&alternate_answers) {
                for r in 0..9 {
                    for c in 0..9 {
                        answers[r][c] = alternate_answers[r][c];
                    }
                }
                return false;
            }
        }
    }

    return found_at_least_one_answer;
}

pub fn solve_sudoku(board: [[u8; 9]; 9]) -> Vec<Vec<u8>> {
    let mut answers = board.map( |v| v.to_vec()).to_vec().clone();
    let mut possible_answers = init_possible_answers(&mut answers);

    while find_one_answer(&mut answers, &mut possible_answers) {}
    answers
}
