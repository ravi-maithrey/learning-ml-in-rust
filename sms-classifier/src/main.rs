#![allow(non_snake_case)]
use reqwest::blocking; // use reqwest::blocking::get to get the dataset
use rustlearn::cross_validation::CrossValidation; // making a cross validation set
use rustlearn::feature_extraction::DictVectorizer; // converting into a bag of words
use rustlearn::linear_models::sgdclassifier; // the actual classfier used
use rustlearn::metrics::accuracy_score; // getting the accuracy metrics
use rustlearn::prelude::*; // rustlearn is for machine learning. prelude contains the requisite lin algebra. models etc are imported as needed
use std::io::{Cursor, Read}; // the reader used in downloading the data
use std::time::Instant;
use zip::read::ZipArchive; // convert the text into bag of words and then so on via one hot encoding

fn download(url: &str) -> Vec<u8> {
    // fn to download the data
    let mut response = blocking::get(url).unwrap(); //get synchronous http data from url
    let mut data = Vec::new(); // initialise a new vector
    response.read_to_end(&mut data).unwrap(); //unwrap the data read in response into the data vector

    data // return the data
}

fn unzip(zip_file: Vec<u8>) -> String {
    // cursor wraps an in-memory buffer to give it a seek implementation. since that is where our downloaded file is we use cursor to unwrap our zip
    let mut archive = ZipArchive::new(Cursor::new(zip_file)).unwrap(); // create an object to read the zip file
    let mut file = archive.by_name("SMSSpamCollection").unwrap(); //SMSSpamCollection is the name of the required file within the zip

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap(); // unwrapping the file into a string

    data
}

// parsing the data which has been converted into a string by unzipping
fn parse(data: &str) -> (SparseRowArray, Array) {
    // initializing the vectoriser
    let mut vectoriser = DictVectorizer::new();
    let mut labels = Vec::new(); // empty vector to store the labels

    // going through each row in the data
    for (row_num, line) in data.lines().enumerate() {
        // in the dataset each label and message is seperated by a tab space so we split them thusly
        let (label, text) = line.split_at(line.find('\t').unwrap());

        // in the new labels vector, put 0 if spam and 1 if not spam (called 'ham' in this dataset [the opposite of spam being ham lol])
        labels.push(match label {
            "spam" => 0.0,
            "ham" => 1.0,
            _ => panic!("Invalid label: {}", label),
        });

        // keep a mapping from token to column index
        for token in text.split_whitespace() {
            vectoriser.partial_fit(row_num, token, 1.0);
        }
    }

    // transform on vectoriser returns our sparse array with one-hot encoding.
    // the other one is the dense array with labels
    (vectoriser.transform(), Array::from(labels))
}

// to fit the data to the pre-decied model, here sgdclassifier
fn fit(X: &SparseRowArray, y: &Array) -> (f32, f32) {
    let num_epochs = 10; // hardcoding this number
    let num_folds = 10; // hardcoded

    //initialization
    let mut train_accuracy = 0.0;
    let mut test_accuracy = 0.0;

    // The cross validation interator returns indices of train and test rows
    for (train_indices, test_indices) in CrossValidation::new(y.rows(), num_folds) {
        // Slice the feature matrices
        let X_train = X.get_rows(&train_indices);
        let X_test = X.get_rows(&test_indices);

        // Slice the target vectors
        let y_train = y.get_rows(&train_indices);
        let y_test = y.get_rows(&test_indices);

        let mut model = sgdclassifier::Hyperparameters::new(X.cols())
            .learning_rate(0.05)
            .l2_penalty(0.01)
            .build();

        // Repeated calls to `fit` perform epochs of training
        for _ in 0..num_epochs {
            model.fit(&X_train, &y_train).unwrap();
        }

        let fold_test_accuracy = accuracy_score(&y_test, &model.predict(&X_test).unwrap());
        let fold_train_accuracy = accuracy_score(&y_train, &model.predict(&X_train).unwrap());

        test_accuracy += fold_test_accuracy;
        train_accuracy += fold_train_accuracy;
    }

    (
        test_accuracy / num_folds as f32,
        train_accuracy / num_folds as f32,
    )
}
fn main() {
    let zipped = download(
        "https://archive.ics.uci.edu/ml/machine-learning-databases/00228/smsspamcollection.zip",
    );
    let raw_data = unzip(zipped);

    let (X, y) = parse(&raw_data);

    println!(
        "X: {} rows, {} columns, {} non-zero entries Y: {:.2}% positive class",
        X.rows(),
        X.cols(),
        X.nnz(),
        y.mean() * 100.0
    );


    let start_time = Instant::now();
    let (test_accuracy, train_accuracy) = fit(&X, &y);
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("Training time: {:?} seconds",
             duration);
    println!("Test accuracy: {:.3}, train accuracy: {:.3}",
             test_accuracy, train_accuracy);
}
