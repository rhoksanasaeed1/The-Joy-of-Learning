# A Sample of 2000 Lines of R Code

# Setup

# Define variables
A<-1
B<-10
C<-5

# Create a matrix
M<-matrix(data=c(A,B,C,A,B,C), nrow=3, ncol=2, byrow=T)

# Create a data frame
DF<-data.frame(V1=c(A,B,C), V2=c(A,B,C))

# Create basic summary statistics
summary(DF)

# Create a scatterplot
plot(DF$V1, DF$V2, main="Scatterplot of V1 and V2",
     xlab="V1", ylab="V2", pch=20, type="b", col="green")

# Load additional packages
library(forecast)
library(ggplot2)

# Create a random sample
x<-rnorm(1000)

# Calculate summary statistics
mean(x)
median(x)
sd(x)

# Graph a histogram
hist(x,main="Histogram of x", col="green")

# Fit a linear model
lm1<-lm(x~1)
summary(lm1)

# Fit a smoothing spline
spl1<-smooth.spline(x)
spl1

# Create a density plot
plot(density(x),main="Density Plot of x", col="green")

# Add a regression line
abline(lm1, col="red")

# Run an ARIMA model
fit <- auto.arima(x)
fit

# Forecast future values
forecast <- forecast(fit, h=10)
plot(forecast,main = 'Forecast of x', col="green")

# Graph a box plot
boxplot(x, main="Boxplot of x", col="green")

# Add a trend line
abline(lm(x~1), col="red")

# Generate a qqnorm plot
qqnorm(x, main="QQNorm of x", col="green")

# Add a trendline
abline(lm(x~1), col="red")

# Generate a correlation coefficient
cor(x)

# Generate a scatterplot
plot(x, main="Scatterplot of x", col="green")

# Create a linear regression model
lm2 <- lm(x~1)
summary(lm2)

# Plot a time serries
time_series <- ts(x)
plot(time_series, main="Time Series Plot", col="green")

# Generate a linear trend line
abline(lm(x~1), col="red")

# Create a dummy variables
dummy <- ifelse(x > 0,1,0)

# Generate a scatterplot of the dummy variables
plot(x,dummy,main="Scatterplot of Dummy Variables", col="green")

# Add a linear regression line
abline(lm(dummy~x), col="red")

# Generate a linear regression model
lm3<-lm(dummy~x)
summary(lm3)

# Generate a density plot
plot(density(dummy), main="Density Plot of Dummy Variables", col="green")

# Generate a violin plot
violinplot(x, col="green")

# Generate a time serries
time_series2 <- ts(dummy)
plot(time_series2,main="Time Series Plot", col="green")

# Create bootstrapped means
mean_boot <- boot.mean(x, R=1000)

# Create boxplot of the bootstrapped means
boxplot(mean_boot, col="green")

# Create a general linear model
glm1<-glm(dummy~x)
summary(glm1)

# Create a qqnorm plot
qqnorm(dummy,main="QQNorm of Dummy Variables", col="green")

# Generate a logistic regression model
glm2<-glm(dummy~x, family=binomial(link="logit"))
summary(glm2)

# Create a contingency table
table1 <- table(x,dummy)
table1

# Create a heatmap
heatmap(table1, col="green")

# Create a probability plot
probplot(x, main="Probability Plot of x", col="green")

# Generate a poisson regression
glm3<-glm(dummy~x, family="poisson")
summary(glm3)

# Generate a linear discriminant analysis
lda1<-lda(dummy~x)

# Create a scatterplot matrix
pairs(DF, main="Scatterplot Matrix", col="green")

# Generate a regression tree
rt1<-rpart(dummy~x)

# Get the accuracy of the regression tree
predict(rt1, type="class")

# Get the confusion matrix
confmat<-table(rt1$y, predict(rt1, type="class"))
confmat

# Create a barplot
barplot(table1, main="Barplot of x and Dummy", col="green")

# Create a PCA
pca1<-princomp(DF)
summary(pca1)

# Generate a scree plot
plot(pca1, type="lines",main="Scree Plot of PCA", col="green")

# Generate a scatterplot
plot(pca1$scores, type="points",main="Scatter Plot of PCA", col="green")

# Get the loadings
loadings(pca1)

# Create a 3-D PCA scatterplot
scatterplot3d(pca1$scores, main="3D Scatterplot of PCA Scores", col="green")

# Generate a heatmap
heatmap(pca1$scores, main="Heatmap of PCA Scores", col="green")

# Generate a hierarchical clustering 
hc<-hclust(dist(DF))

# Plot the hierarchical clustering
plot(hc, col="green")

# Generate a K-Means clustering
km<-kmeans(DF, centers=3)

# Get the cluster centers
km$centers

# Get the clustering results
km$cluster

# Plot the K-Means clustering
plot(DF,main="K-Means Clustering of DF", col="green")
points(km$centers, pch=15, col="red")

# Get the silhouette coefficient
Sil_coef <- silhouettes(DF,km$cluster)
Sil_coef

# Get the adjusted Rand index
Adjusted_Rand <- adjustedRand(km$cluster, km$cluster)
Adjusted_Rand

# Create a NMF model
nmf1<-nmf(DF, k=3)

# Get the models features
nmf1$V

# Plot the W and H matrices
W <- plot(nmf1$W, type="l", ylim=c(0,1),main="W Matrix for NMF", col="green")
H <- plot(nmf1$H, type="l", ylim=c(0,1),main="H Matrix for NMF", col="green")

# Create a fuzzy clustering model
fm <- Fanny(DF, k=3)

# Get the cluster centers
fm$centroids

# Plot the fuzzy clustering
plot(fm,main="Fuzzy Clustering of DF", col="green")
points(fm$centroids, pch=15, col="red")

# Create an SVM model
svm1<-svm(DF$V1~DF$V2)

# Get the model's coefficients
svm1$coef0

# Plot the SVM
plot(svm1, DF$V1, DF$V2, main="Support Vector Machine", col="green")

# Generate a MDS model
mds1<-cmdscale(dist(DF))

# Plot the MDS
plot(mds1, type="n", main="Multidimensional Scaling of DF", col="green")

# Generate a random forest
rf1<-randomForest(dummy~x)

# Get the model's importance
rf1$importance 

# Plot the random forest
plot(rf1, main="Random Forest", col="green")