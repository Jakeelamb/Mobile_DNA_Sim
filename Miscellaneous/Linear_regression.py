import numpy as np
import pandas as pd
from sklearn.model_selection import train_test_split
from sklearn.linear_model import LinearRegression
from sklearn.metrics import mean_squared_error, r2_score
import matplotlib.pyplot as plt
from sklearn.preprocessing import PolynomialFeatures
import seaborn as sns

# Load your data (replace this with your actual data loading method)
data = pd.read_csv('/home/jake/Projects/exon_ranges_summary.csv')


# Visualize distributions
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(15, 5))
sns.histplot(data['Genome_size'], kde=True, ax=ax1)
ax1.set_title('Distribution of Genome Size')
ax1.set_xlabel('Genome Size')
sns.histplot(data['Exon_number_sum'], kde=True, ax=ax2)
ax2.set_title('Distribution of Number of Exons')
ax2.set_xlabel('Number of Exons')
plt.tight_layout()
plt.show()


data['log_genome_size'] = np.log(data['Genome_size'])
data['log_number_of_exons'] = np.log(data['Exon_number_sum'])


# Visualize log-transformed distributions
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(15, 5))
sns.histplot(data['log_genome_size'], kde=True, ax=ax1)
ax1.set_title('Distribution of Log Genome Size')
ax1.set_xlabel('Log Genome Size')
sns.histplot(data['log_number_of_exons'], kde=True, ax=ax2)
ax2.set_title('Distribution of Log Number of Exons')
ax2.set_xlabel('Log Number of Exons')
plt.tight_layout()
plt.show()

# Scatter plots
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(15, 5))
ax1.scatter(data['Genome_size'], data['Exon_number_sum'], alpha=0.5)
ax1.set_xlabel('Genome Size')
ax1.set_ylabel('Number of Exons')
ax1.set_title('Genome Size vs Number of Exons')

ax2.scatter(data['log_genome_size'], data['log_number_of_exons'], alpha=0.5)
ax2.set_xlabel('Log Genome Size')
ax2.set_ylabel('Log Number of Exons')
ax2.set_title('Log Genome Size vs Log Number of Exons')
plt.tight_layout()
plt.show()

# Prepare the features (X) and target (y) using log-transformed data
X = data['log_genome_size'].values.reshape(-1, 1)
y = data['log_number_of_exons'].values

# Split the data into training and testing sets
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

# Create and train the model
model = LinearRegression()
model.fit(X_train, y_train)

# Make predictions on the test set
y_pred = model.predict(X_test)

# Evaluate the model
mse = mean_squared_error(y_test, y_pred)
r2 = r2_score(y_test, y_pred)

print(f"Mean squared error (on log scale): {mse}")
print(f"R-squared score (on log scale): {r2}")

# Visualize the results
plt.figure(figsize=(10, 6))
plt.scatter(X_test, y_test, color='blue', alpha=0.5, label='Actual')
plt.plot(X_test, y_pred, color='red', label='Predicted')
plt.xlabel('Log Genome Size')
plt.ylabel('Log Number of Exons')
plt.title('Log-transformed Exon Prediction Model')
plt.legend()
plt.show()