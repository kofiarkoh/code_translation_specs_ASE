#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <stdbool.h>

#define lli long long int
#define min(a,b) ((a)<(b)?(a):(b))
#define max(a,b) ((a)>(b)?(a):(b))

bool has(int i,int *d,int n){
	int j;
	for(j=1;j<=n;j++){
		if(i==d[j]){
			return true;
		}
	}
	return false;
}


int solve(int n, int *num,int num_length){
	int i;
	int j;
	int a;
	int res = 0;
	for(i=4;i>=0;i--){
		a = (n / (int)pow(10,i)) % 10;
		if(has(a,num,num_length)){
			res += a * pow(10,i);
		}else{
			j = 0;
			while(a > num[j])j++;
			res += num[j] * pow(10,i);
			break;
		}
	}
	i--;
	for(;i>=0;i--){
		res += num[0] * pow(10,i);
	}
	return res;
}

int main(void){
	int i,j;
	int n,k;
	int *d;
	int *num;
	int num_length = 0;
	scanf("%d%d",&n,&k);
	d = calloc(sizeof(int),k+1);
	num = calloc(sizeof(int),10);
	for(i=1;i<=k;i++){
		scanf("%d",&d[i]);
	}
	for(i=0;i<=9;i++){
		if(!has(i,d,k)){
			num[num_length++] = i;
		}
	}
	printf("%d\n", solve(n,num,num_length));


	return 0;
}