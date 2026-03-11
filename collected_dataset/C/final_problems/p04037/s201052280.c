#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>
 
#define INF 1000000000
 
#define min(a,b) (((a)<(b))?(a):(b))
#define max(a,b) (((a)>(b))?(a):(b))

int main() {
	int N;
	int a[100001];
	int i, j;
	int odd = 0, even = 0;
	int f = -1;
	
	scanf("%d", &N);
	for(i=1;i<=N;i++) {
		scanf("%d", &a[i]);
		if(a[i] % 2) odd++;
		else even++;
		if(a[i] == 1) f++;
	}
	
	if(N % 2 == 0) {
		if(even % 2 == 0) {
			if(odd == 0) {
				printf("Second\n");
			} else {
				printf("First\n");
			}
		} else {
			printf("First\n");
		}
	} else {
		if(even % 2 == 0) {
			if(even == 0) {
				printf("Second\n");
			} else {
				printf("First\n");
			}
		} else {
			if(f > 0) {
				printf("First\n");
			} else {
				printf("Second\n");
			}
		}
	}
	
	return 0;
}