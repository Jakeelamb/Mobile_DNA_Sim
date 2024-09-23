import pandas as pd
import os 

df = pd.read_csv("/home/jake/Projects/te_sim/Data/exon_ranges_summary.csv")
def filt_df(df):
    # Create new columns called Species_name and Assembly
    df['Species_name'] = df.iloc[:, 0].apply(lambda x: x.split(".")[0])
    df['Assembly'] = df.iloc[:, 0].apply(lambda x: '.'.join(x.split(".")[1:]))
    
    # Create a filtered dataframe that contains the Species_name column, Assembly, Genome_size, and Average Range
    filtered_df = df[['Species_name', 'Assembly', 'Genome_size', 'Average Range']]
    # rename Average Range to Num_exons
    filtered_df.rename(columns={"Average Range": "Num_exons"}, inplace=True)
    return filtered_df

filtered_data = filt_df(df)

# write filtered_data to csv at /home/jake/Projects/te_sim/Data/Species_data.csv
filtered_data.to_csv("/home/jake/Projects/te_sim/Data/Species_data.csv", index=False)

print(filtered_data)

