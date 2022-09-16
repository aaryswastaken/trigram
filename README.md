# Trigram

### This program is using `blogtext.csv` coming from [Kaggle's Blog Authorship Corpus](https://www.kaggle.com/datasets/rtatman/blog-authorship-corpus) dataset 

Python: `blogtext.csv -> temp.txt` and rust: `temp.txt -> frequencies.txt`

The aim is to generate a list of frequencies of every trigrams in the english language using some freely available data from the internet, this is why this project is composed of two parts: 
 - The python script is written such that it is specific to the source (here the kaggle dataset) and normalise it
 - The rust script takes the normalised data and computes the frequency

Both of these scripts are heavily unoptimized because they were written to quickly get these data from a rather large dataset and should be used with precaution when working on large amounts of data because it will take a hell lot of time

A future improvement would be to scrape the english dataset directly from [en.wikipedia.org](https://en.wikipedia.org) using some automated rust script and directly process it rather than saving it in a very bad manner in a `temp.txt` file. I am aware of these issues and possible improvements and might apply them in the **SOON (TM)** future