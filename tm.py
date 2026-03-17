import pandas as pd

df = pd. read_csv('r_code/run_reports/run_summary.csv')
print(df['status'].value_counts())
print(df['compiled'].value_counts())
print(df['compiled'].value_counts())