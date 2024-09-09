import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
from sklearn.model_selection import train_test_split
from sklearn.linear_model import LinearRegression
from sklearn.metrics import mean_squared_error, r2_score
from sklearn.preprocessing import PolynomialFeatures

# Load your data
data = pd.read_csv('/home/jake/Projects/exon_ranges_summary.csv')

# Function to plot distributions
def plot_distributions(data, x1, x2, title1, title2, xlabel1, xlabel2):
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(15, 5))
    sns.histplot(data[x1], kde=True, ax=ax1)
    ax1.set_title(title1)
    ax1.set_xlabel(xlabel1)
    sns.histplot(data[x2], kde=True, ax=ax2)
    ax2.set_title(title2)
    ax2.set_xlabel(xlabel2)
    plt.tight_layout()
    plt.show()

# Original distributions
plot_distributions(data, 'Genome_size', 'Exon_number_sum', 
                   'Distribution of Genome Size', 'Distribution of Number of Exons',
                   'Genome Size', 'Number of Exons')

# Log-transformed distributions
data['log_genome_size'] = np.log(data['Genome_size'])
data['log_number_of_exons'] = np.log(data['Exon_number_sum'])
plot_distributions(data, 'log_genome_size', 'log_number_of_exons', 
                   'Distribution of Log Genome Size', 'Distribution of Log Number of Exons',
                   'Log Genome Size', 'Log Number of Exons')

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

# Binning
def bin_data(data, n_bins=20):
    data_binned = data.copy()
    data_binned['genome_size_bin'] = pd.cut(data_binned['Genome_size'], bins=n_bins)
    binned = data_binned.groupby('genome_size_bin')['Exon_number_sum'].mean().reset_index()
    binned['genome_size_bin_mid'] = binned['genome_size_bin'].apply(lambda x: x.mid)
    return binned

# After binning the data
binned_data = bin_data(data)

# Remove rows with NaN values
binned_data = binned_data.dropna()

# Then proceed with your model fitting
X_binned = binned_data['genome_size_bin_mid'].values.reshape(-1, 1)
y_binned = binned_data['Exon_number_sum'].values

model_binned = LinearRegression()
model_binned.fit(X_binned, y_binned)

plt.figure(figsize=(10, 6))
plt.scatter(data['Genome_size'], data['Exon_number_sum'], alpha=0.5, label='Original Data')
plt.scatter(binned_data['genome_size_bin_mid'], binned_data['Exon_number_sum'], color='red', s=50, label='Binned Data')
plt.xlabel('Genome Size')
plt.ylabel('Number of Exons')
plt.title('Original vs Binned Data')
plt.legend()
plt.show()

# Train model on binned data
X_binned = binned_data['genome_size_bin_mid'].values.reshape(-1, 1)
y_binned = binned_data['Exon_number_sum'].values

model_binned = LinearRegression()
model_binned.fit(X_binned, y_binned)

plt.figure(figsize=(10, 6))
plt.scatter(binned_data['genome_size_bin_mid'], binned_data['Exon_number_sum'], color='blue', label='Binned Data')
plt.plot(X_binned, model_binned.predict(X_binned), color='red', label='Predicted')
plt.xlabel('Genome Size')
plt.ylabel('Number of Exons')
plt.title('Binned Data Model')
plt.legend()
plt.show()


from sklearn.utils import compute_sample_weight

# Weighted sampling
X = data['log_genome_size'].values.reshape(-1, 1)
y = data['log_number_of_exons'].values

# Compute weights (inverse of frequency)
sample_weights = compute_sample_weight(class_weight='balanced', y=y)

# Train weighted model
X_train, X_test, y_train, y_test, weights_train, _ = train_test_split(X, y, sample_weights, test_size=0.2, random_state=42)

model_weighted = LinearRegression()
model_weighted.fit(X_train, y_train, sample_weight=weights_train)

y_pred = model_weighted.predict(X_test)

plt.figure(figsize=(10, 6))
plt.scatter(X_test, y_test, c=compute_sample_weight(class_weight='balanced', y=y_test), cmap='viridis', alpha=0.5)
plt.plot(X_test, y_pred, color='red', label='Predicted')
plt.xlabel('Log Genome Size')
plt.ylabel('Log Number of Exons')
plt.title('Weighted Sampling Model')
plt.colorbar(label='Sample Weight')
plt.legend()
plt.show()

print(f"R-squared score (Weighted): {r2_score(y_test, y_pred)}")

from sklearn.utils import compute_sample_weight

# Weighted sampling
X = data['log_genome_size'].values.reshape(-1, 1)
y = data['log_number_of_exons'].values

# Compute weights (inverse of frequency)
sample_weights = compute_sample_weight(class_weight='balanced', y=y)

# Train weighted model
X_train, X_test, y_train, y_test, weights_train, _ = train_test_split(X, y, sample_weights, test_size=0.2, random_state=42)

model_weighted = LinearRegression()
model_weighted.fit(X_train, y_train, sample_weight=weights_train)

y_pred = model_weighted.predict(X_test)

plt.figure(figsize=(10, 6))
plt.scatter(X_test, y_test, c=compute_sample_weight(class_weight='balanced', y=y_test), cmap='viridis', alpha=0.5)
plt.plot(X_test, y_pred, color='red', label='Predicted')
plt.xlabel('Log Genome Size')
plt.ylabel('Log Number of Exons')
plt.title('Weighted Sampling Model')
plt.colorbar(label='Sample Weight')
plt.legend()
plt.show()

print(f"R-squared score (Weighted): {r2_score(y_test, y_pred)}")

from sklearn.neighbors import KernelDensity

# KDE
X = data['log_genome_size'].values.reshape(-1, 1)
y = data['log_number_of_exons'].values

kde = KernelDensity(kernel='gaussian', bandwidth=0.2).fit(X)
X_kde = np.linspace(X.min(), X.max(), 1000)[:, np.newaxis]
log_dens = kde.score_samples(X_kde)

plt.figure(figsize=(10, 6))
plt.scatter(X, y, alpha=0.5)
plt.plot(X_kde, np.exp(log_dens), color='red')
plt.xlabel('Log Genome Size')
plt.ylabel('Log Number of Exons / Density')
plt.title('KDE of Log Genome Size')
plt.show()

# Generate samples from KDE
X_sampled = kde.sample(n_samples=len(X), random_state=42)
y_sampled = np.interp(X_sampled.ravel(), X.ravel(), y)

plt.figure(figsize=(10, 6))
plt.scatter(X, y, alpha=0.5, label='Original Data')
plt.scatter(X_sampled, y_sampled, alpha=0.5, color='red', label='KDE Sampled Data')
plt.xlabel('Log Genome Size')
plt.ylabel('Log Number of Exons')
plt.title('Original vs KDE Sampled Data')
plt.legend()
plt.show()

# Train model on KDE sampled data
X_train, X_test, y_train, y_test = train_test_split(X_sampled, y_sampled, test_size=0.2, random_state=42)

model_kde = LinearRegression()
model_kde.fit(X_train, y_train)

y_pred = model_kde.predict(X_test)

plt.figure(figsize=(10, 6))
plt.scatter(X_test, y_test, color='blue', alpha=0.5, label='Actual')
plt.plot(X_test, y_pred, color='red', label='Predicted')
plt.xlabel('Log Genome Size')
plt.ylabel('Log Number of Exons')
plt.title('KDE Sampled Model')
plt.legend()
plt.show()

print(f"R-squared score (KDE): {r2_score(y_test, y_pred)}")

from sklearn.neighbors import KernelDensity

# KDE
X = data['log_genome_size'].values.reshape(-1, 1)
y = data['log_number_of_exons'].values

kde = KernelDensity(kernel='gaussian', bandwidth=0.2).fit(X)
X_kde = np.linspace(X.min(), X.max(), 1000)[:, np.newaxis]
log_dens = kde.score_samples(X_kde)

plt.figure(figsize=(10, 6))
plt.scatter(X, y, alpha=0.5)
plt.plot(X_kde, np.exp(log_dens), color='red')
plt.xlabel('Log Genome Size')
plt.ylabel('Log Number of Exons / Density')
plt.title('KDE of Log Genome Size')
plt.show()

# Generate samples from KDE
X_sampled = kde.sample(n_samples=len(X), random_state=42)
y_sampled = np.interp(X_sampled.ravel(), X.ravel(), y)

plt.figure(figsize=(10, 6))
plt.scatter(X, y, alpha=0.5, label='Original Data')
plt.scatter(X_sampled, y_sampled, alpha=0.5, color='red', label='KDE Sampled Data')
plt.xlabel('Log Genome Size')
plt.ylabel('Log Number of Exons')
plt.title('Original vs KDE Sampled Data')
plt.legend()
plt.show()

# Train model on KDE sampled data
X_train, X_test, y_train, y_test = train_test_split(X_sampled, y_sampled, test_size=0.2, random_state=42)

model_kde = LinearRegression()
model_kde.fit(X_train, y_train)

y_pred = model_kde.predict(X_test)

plt.figure(figsize=(10, 6))
plt.scatter(X_test, y_test, color='blue', alpha=0.5, label='Actual')
plt.plot(X_test, y_pred, color='red', label='Predicted')
plt.xlabel('Log Genome Size')
plt.ylabel('Log Number of Exons')
plt.title('KDE Sampled Model')
plt.legend()
plt.show()

print(f"R-squared score (KDE): {r2_score(y_test, y_pred)}")

from sklearn.cluster import KMeans

# Piece-wise modeling
X = data['log_genome_size'].values.reshape(-1, 1)
y = data['log_number_of_exons'].values

# Use KMeans to split the data into two clusters
kmeans = KMeans(n_clusters=2, random_state=42)
clusters = kmeans.fit_predict(X)

# Create separate models for each cluster
model1 = LinearRegression()
model2 = LinearRegression()

model1.fit(X[clusters == 0], y[clusters == 0])
model2.fit(X[clusters == 1], y[clusters == 1])

# Predict using the piece-wise model
def predict_piecewise(X):
    predictions = np.zeros(X.shape[0])
    cluster_pred = kmeans.predict(X)
    predictions[cluster_pred == 0] = model1.predict(X[cluster_pred == 0])
    predictions[cluster_pred == 1] = model2.predict(X[cluster_pred == 1])
    return predictions

# Visualize the piece-wise model
plt.figure(figsize=(10, 6))
plt.scatter(X, y, c=clusters, cmap='viridis', alpha=0.5)
X_sort = np.sort(X, axis=0)
y_pred = predict_piecewise(X_sort)
plt.plot(X_sort, y_pred, color='red', linewidth=2)
plt.xlabel('Log Genome Size')
plt.ylabel('Log Number of Exons')
plt.title('Piece-wise Model')
plt.colorbar(label='Cluster')
plt.show()

# Evaluate the piece-wise model
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)
y_pred = predict_piecewise(X_test)
print(f"R-squared score (Piece-wise): {r2_score(y_test, y_pred)}")

from sklearn.linear_model import HuberRegressor, RANSACRegressor

# Robust regression
X = data['log_genome_size'].values.reshape(-1, 1)
y = data['log_number_of_exons'].values

X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

# Huber Regression
huber = HuberRegressor()
huber.fit(X_train, y_train)
y_pred_huber = huber.predict(X_test)

# RANSAC Regression
ransac = RANSACRegressor(random_state=42)
ransac.fit(X_train, y_train)
y_pred_ransac = ransac.predict(X_test)

# Visualize results
plt.figure(figsize=(12, 6))
plt.scatter(X_test, y_test, alpha=0.5, label='Actual')
plt.plot(X_test, y_pred_huber, color='red', label='Huber Regression')
plt.plot(X_test, y_pred_ransac, color='green', label='RANSAC Regression')
plt.xlabel('Log Genome Size')
plt.ylabel('Log Number of Exons')
plt.title('Robust Regression Models')
plt.legend()
plt.show()

print(f"R-squared score (Huber): {r2_score(y_test, y_pred_huber)}")
print(f"R-squared score (RANSAC): {r2_score(y_test, y_pred_ransac)}")

from sklearn.tree import DecisionTreeRegressor
from sklearn.ensemble import RandomForestRegressor, GradientBoostingRegressor

# Non-parametric models
X = data['log_genome_size'].values.reshape(-1, 1)
y = data['log_number_of_exons'].values

X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

# Decision Tree
dt = DecisionTreeRegressor(random_state=42)
dt.fit(X_train, y_train)
y_pred_dt = dt.predict(X_test)

# Random Forest
rf = RandomForestRegressor(n_estimators=100, random_state=42)
rf.fit(X_train, y_train)
y_pred_rf = rf.predict(X_test)

# Gradient Boosting
gb = GradientBoostingRegressor(n_estimators=100, random_state=42)
gb.fit(X_train, y_train)
y_pred_gb = gb.predict(X_test)

# Visualize results
plt.figure(figsize=(12, 6))
plt.scatter(X_test, y_test, alpha=0.5, label='Actual')
plt.plot(X_test, y_pred_dt, color='red', label='Decision Tree')
plt.plot(X_test, y_pred_rf, color='green', label='Random Forest')
plt.plot(X_test, y_pred_gb, color='blue', label='Gradient Boosting')
plt.xlabel('Log Genome Size')
plt.ylabel('Log Number of Exons')
plt.title('Non-parametric Models')
plt.legend()
plt.show()

print(f"R-squared score (Decision Tree): {r2_score(y_test, y_pred_dt)}")
print(f"R-squared score (Random Forest): {r2_score(y_test, y_pred_rf)}")
print(f"R-squared score (Gradient Boosting): {r2_score(y_test, y_pred_gb)}")

# Collect R-squared scores from all models
models = {
    'Original Linear Regression': model1,
    'Binned Data': model_binned,
    'Weighted Sampling': model_weighted,
    'KDE Sampled': model_kde,
    'Quantile Transformed': model_kde,
    'Piece-wise': None,  # We'll add this manually
    'Huber Regression': huber,
    'RANSAC Regression': ransac,
    'Decision Tree': dt,
    'Random Forest': rf,
    'Gradient Boosting': gb
}

# Function to calculate R-squared score
def get_r2_score(model, X, y):
    if model is None:
        return None  # For piece-wise model
    return r2_score(y, model.predict(X))

# Calculate R-squared scores
r2_scores = {name: get_r2_score(model, X_test, y_test) for name, model in models.items()}
r2_scores['Piece-wise'] = r2_score(y_test, predict_piecewise(X_test))

# Sort models by R-squared score
sorted_models = sorted(r2_scores.items(), key=lambda x: x[1] if x[1] is not None else -1, reverse=True)

# Print sorted R-squared scores
print("Model Performance Summary (R-squared scores):")
for name, score in sorted_models:
    print(f"{name}: {score:.4f}")

# Visualize model performance
plt.figure(figsize=(12, 6))
plt.bar(range(len(sorted_models)), [score for _, score in sorted_models])
plt.xticks(range(len(sorted_models)), [name for name, _ in sorted_models], rotation=45, ha='right')
plt.ylabel('R-squared Score')
plt.title('Model Performance Comparison')
plt.tight_layout()
plt.show()


