import pandas as pd

df_input_food = pd.read_csv("input_food.csv")
print(df_input_food.head())

df_food = pd.read_csv("food.csv")
print(df_food.describe())
print(df_food.head())

