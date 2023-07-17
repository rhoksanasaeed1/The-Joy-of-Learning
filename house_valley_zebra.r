# Download and load the dataset

# Read the data

df <- read.csv("the_joy_of_learning.csv", header = TRUE)

# Remove missing values

df <- na.omit(df)

# Summarize the data

df_summary <- summary(df)

# Check the structure of the data

str(df)

# Explore the distribution of the data

# Histogram of marks

hist(df$marks)

# Frequency distribution of marks

freq_dist_marks <- table(df$marks)

# Bar plot of marks

barplot(freq_dist_marks)

# Boxplot of marks

boxplot(df$marks, main = "Marks Distribution")

# Time series plot of marks

plot(df$marks, type = "l",
     main = "Time series plot of Marks")

# Correlation between marks

correlation_of_marks <- cor(df$marks)

# Scatter plot of marks

plot(df$marks ~ df$time, type="p",
     pch=19, col="darkgreen",
     main="Scatter plot of Marks",
     xlab="Time", ylab="Marks")

# Linear regression of marks

regression_of_marks <- lm(df$marks ~ df$time, data = df)

# Summary of linear regression

summary(regression_of_marks)

# Residuals plot of linear regression

plot(regression_of_marks, which = 1,
     main = "Residuals plot")

# Predictions using linear regression

predictions_using_regression <- predict(regression_of_marks)

# Plot of predictions

plot(predictions_using_regression,type="l",
     col="red",main="Predictions using Linear Regression")

# Descriptive statistics

descriptive_statistics <- summary(df)

# Summary of data

summary(df)

# Correlation of marks

correlation_of_marks <- cor(df$marks)

# Correlation between marks and time

correlation_between_marks_time <- cor(df$marks, df$time)

# Variance of marks

variance_of_marks <- var(df$marks)

# Mean of marks

mean_of_marks <- mean(df$marks)

# Range of marks

range_of_marks <- range(df$marks)

# Maximum of marks

maximum_of_marks <- max(df$marks)

# Minimum of marks

minimum_of_marks <- min(df$marks)

# Standard deviation of marks

standard_deviation_of_marks <- sd(df$marks)

# Kendall correlation coefficient of marks

kendall_correlation_coefficient_of_marks <- cor.test(df$marks, df$time, method = "kendall")

# Spearman correlation coefficient of marks

spearman_correlation_coefficient_of_marks <- cor.test(df$marks, df$time, method = "spearman")

# Shapiro Wilk test of marks

shapiro_wilk_test_of_marks <- shapiro.test(df$marks)

# Levene test of marks

levene_test_of_marks <- leveneTest(df$marks)

# Kolmogorov Smirnov test of marks

kolmogorov_smirnov_test_of_marks <- ks.test(df$marks)

# Analysis of variance

anova_of_marks <- aov(df$marks ~ df$time)

# Summary of analysis of variance

summary(anova_of_marks)

# Bartlett test

bartlett_test_of_marks <- bartlett.test(df$marks, df$time)

# Two-sample t-test

two_sample_t_test_of_marks <- t.test(df$marks, df$time)

# Summary of two-sample t-test

summary(two_sample_t_test_of_marks)

# Analysis of Covariance

analysis_of_covariance_of_marks <- aov(df$marks ~ df$time + df$gender)

# Summary of analysis of covariance

summary(analysis_of_covariance_of_marks)

# Data transformation

# Log transformation

log_transformation_of_marks <- log(df$marks)

# Square root transformation

square_root_transformation_of_marks <- sqrt(df$marks)

# Box Cox transformation

library(MASS)

box_cox_transformation_of_marks <- boxcox(df$marks)

# Prediction using linear regression

prediction_of_marks <- predict(regression_of_marks, newdata = df)

# Plot of prediction

plot(prediction_of_marks, type="l",
     col="blue", main="Prediction using Linear Regression")

# Test of linearity

test_of_linearity <- rlm(df$marks ~ df$time)

# Summary of test of linearity

summary(test_of_linearity)

# Summary of dataset

summary(df)