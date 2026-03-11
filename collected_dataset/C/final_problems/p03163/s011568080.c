#include <stdio.h>
#include <math.h>
#include <stdlib.h>
 
long long int dp[100][100010];
long long int w[100];
long long int v[100];
 
 
long long int max(long long int a, long long int b) {
	if (a < b)
		return b;
	else
		return a;
}
 
int main() {
 
	double maxweight = pow(10, 5);
	double maxvalue = pow(10, 9);
	int item = 0;
	int capacity = 0;
	int i = 0;
	int j = 0;
	long long int value = 0;
 
	if (scanf("%d", &item) > 0) {
		if (0 >= item || item > 100)
			exit(1);
	}
	if (scanf("%d", &capacity) > 0) {
		if (0 >= capacity || capacity > maxweight)
			exit(1);
	}
 
	for (i = 0; i < item; i++)
	{
		if (scanf("%lld", &w[i]) > 0) {
			if (0 >= w[i] || w[i] > maxweight)
				exit(1);
		}
		if (scanf("%lld", &v[i]) > 0) {
			if (0 >= v[i] || v[i] > maxvalue)
				exit(1);
		}
	}
 
	for (i = 0; i < item; i++) {
		for (j = 0; j < capacity; j++) {
			dp[i][j] = 0;
		}
	}
 
	for (i = 0; i < item; i++) {
		for (j = 0; j < capacity + 1; j++) {
			if (w[i] <= j)
			{
				dp[i + 1][j] = max(dp[i][j - w[i]] + v[i], dp[i][j]);
			}
			else
				dp[i + 1][j] = dp[i][j];
		}
	}
  
	for (i = 0; i < capacity; i++) {
		value = max(dp[item][i], dp[item][i + 1]);
	}
		
	printf("%lld", value);
}
 