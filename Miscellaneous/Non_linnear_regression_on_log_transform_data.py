import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import PolynomialFeatures
from sklearn.linear_model import LinearRegression
from sklearn.svm import SVR
from sklearn.ensemble import RandomForestRegressor
from sklearn.metrics import r2_score
from sklearn.neural_network import MLPRegressor
from scipy.stats import spearmanr

# Load your data (assuming it's already loaded as 'data')
# If not, uncomment the following line:
data = pd.read_csv('/home/jake/Projects/exon_ranges_summary.csv')

X = data['Genome_size'].values.reshape(-1, 1)
y = data['Exon_number_sum'].values

# 1. Scatter plot with log scales
plt.figure(figsize=(10, 6))
plt.scatter(X, y, alpha=0.5)
plt.xscale('log')
plt.yscale('log')
plt.xlabel('Genome Size (log scale)')
plt.ylabel('Number of Exons (log scale)')
plt.title('Genome Size vs Number of Exons (Log-Log Plot)')
plt.show()

# 2. Polynomial Regression
degrees = [2, 3, 4]
plt.figure(figsize=(15, 5))

for i, degree in enumerate(degrees, 1):
    poly_features = PolynomialFeatures(degree=degree, include_bias=False)
    X_poly = poly_features.fit_transform(X)
    
    X_train, X_test, y_train, y_test = train_test_split(X_poly, y, test_size=0.2, random_state=42)
    
    model = LinearRegression()
    model.fit(X_train, y_train)
    
    y_pred = model.predict(X_test)
    r2 = r2_score(y_test, y_pred)
    
    plt.subplot(1, 3, i)
    plt.scatter(X, y, alpha=0.5)
    X_plot = np.linspace(X.min(), X.max(), 100).reshape(-1, 1)
    X_plot_poly = poly_features.transform(X_plot)
    y_plot = model.predict(X_plot_poly)
    plt.plot(X_plot, y_plot, color='r')
    plt.xlabel('Genome Size')
    plt.ylabel('Number of Exons')
    plt.title(f'Polynomial Regression (degree={degree})\nR² = {r2:.4f}')

plt.tight_layout()
plt.show()

# 3. Support Vector Regression
svr = SVR(kernel='rbf')
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)
svr.fit(X_train, y_train)
y_pred_svr = svr.predict(X_test)
r2_svr = r2_score(y_test, y_pred_svr)

plt.figure(figsize=(10, 6))
plt.scatter(X, y, alpha=0.5)
X_plot = np.linspace(X.min(), X.max(), 100).reshape(-1, 1)
y_plot = svr.predict(X_plot)
plt.plot(X_plot, y_plot, color='r')
plt.xlabel('Genome Size')
plt.ylabel('Number of Exons')
plt.title(f'Support Vector Regression\nR² = {r2_svr:.4f}')
plt.show()

# 4. Random Forest Regression
rf = RandomForestRegressor(n_estimators=100, random_state=42)
rf.fit(X_train, y_train)
y_pred_rf = rf.predict(X_test)
r2_rf = r2_score(y_test, y_pred_rf)

plt.figure(figsize=(10, 6))
plt.scatter(X, y, alpha=0.5)
X_plot = np.linspace(X.min(), X.max(), 100).reshape(-1, 1)
y_plot = rf.predict(X_plot)
plt.plot(X_plot, y_plot, color='r')
plt.xlabel('Genome Size')
plt.ylabel('Number of Exons')
plt.title(f'Random Forest Regression\nR² = {r2_rf:.4f}')
plt.show()

# 5. Neural Network (MLP) Regression
mlp = MLPRegressor(hidden_layer_sizes=(100, 50), max_iter=1000, random_state=42)
mlp.fit(X_train, y_train)
y_pred_mlp = mlp.predict(X_test)
r2_mlp = r2_score(y_test, y_pred_mlp)

plt.figure(figsize=(10, 6))
plt.scatter(X, y, alpha=0.5)
X_plot = np.linspace(X.min(), X.max(), 100).reshape(-1, 1)
y_plot = mlp.predict(X_plot)
plt.plot(X_plot, y_plot, color='r')
plt.xlabel('Genome Size')
plt.ylabel('Number of Exons')
plt.title(f'Neural Network Regression\nR² = {r2_mlp:.4f}')
plt.show()

# 6. Spearman Rank Correlation
spearman_corr, _ = spearmanr(X, y)
print(f"Spearman Rank Correlation: {spearman_corr:.4f}")

# 7. Residual Plot (using Random Forest as an example)
y_pred_all = rf.predict(X)
residuals = y - y_pred_all

plt.figure(figsize=(10, 6))
plt.scatter(X, residuals, alpha=0.5)
plt.xlabel('Genome Size')
plt.ylabel('Residuals')
plt.title('Residual Plot (Random Forest)')
plt.axhline(y=0, color='r', linestyle='--')
plt.show()

# Print summary of R-squared scores
print("\nR-squared scores:")
print(f"Polynomial Regression (degree=2): {r2_score(y_test, LinearRegression().fit(PolynomialFeatures(degree=2, include_bias=False).fit_transform(X_train), y_train).predict(PolynomialFeatures(degree=2, include_bias=False).fit_transform(X_test))):.4f}")
print(f"Polynomial Regression (degree=3): {r2_score(y_test, LinearRegression().fit(PolynomialFeatures(degree=3, include_bias=False).fit_transform(X_train), y_train).predict(PolynomialFeatures(degree=3, include_bias=False).fit_transform(X_test))):.4f}")
print(f"Polynomial Regression (degree=4): {r2_score(y_test, LinearRegression().fit(PolynomialFeatures(degree=4, include_bias=False).fit_transform(X_train), y_train).predict(PolynomialFeatures(degree=4, include_bias=False).fit_transform(X_test))):.4f}")
print(f"Support Vector Regression: {r2_svr:.4f}")
print(f"Random Forest Regression: {r2_rf:.4f}")
print(f"Neural Network Regression: {r2_mlp:.4f}")
