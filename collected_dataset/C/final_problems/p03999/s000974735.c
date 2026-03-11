#include<stdio.h>
#include<string.h>
#include<stdlib.h>

int main(){
	char S[11];
	int i, j, k, n, *s;
	unsigned long long *dp, sum, *tensq, *terms;
	scanf("%s", S);
	
	n = (int)strlen(S);
	s = (int *)malloc((unsigned int)n*sizeof(int));
	dp = (unsigned long long *)malloc((unsigned int)(n+1)*sizeof(unsigned long long));
	tensq = (unsigned long long *)malloc((unsigned int)(n)*sizeof(unsigned long long));
	terms = (unsigned long long *)malloc((unsigned int)(n)*sizeof(unsigned long long));
	for(i = 0; i < n; i++) s[i] = (int)(S[i]-'0');
	dp[0] = 0;
	tensq[0] = 1;
	for(i = 1; i < n; i++) tensq[i] = 10*tensq[i-1];
	terms[0] = terms[1] = 1;
	for(i = 2; i < n; i++) terms[i] = 2*terms[i-1];
	
	for(i = 0; i < n; i++){
		dp[i+1] = 0;
		for(j = 0; j <= i; j++){
			sum = 0;
			for(k = 0; k <= j; k++){
				sum += tensq[k]*(unsigned long long)s[i-k];
			}
			dp[i+1] += terms[i-j]*sum+dp[i-j];
		}
	}
	
	printf("%lu\n", dp[n]);
	
	return 0;
}
