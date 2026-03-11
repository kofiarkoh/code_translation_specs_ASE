#include <stdio.h>

void swap(int *a,int *b){
	int tmp;
	tmp = *b;
	*b = *a;
	*a = tmp;
	return;
}

int check(int a[],int n){
	int i;
	for(i=0;i<n;i++){
		if(a[i] == i+1){
			return 0;
		}
	}
	return 1;
}

int main(void){
	int n,m;
	int i;
	int a[100000];
	scanf("%d",&n);
	for(i=0;i<n;i++){
		scanf("%d",&a[i]);
	}
	int sp = 0;
	while(1){
		if(check(a,n) != 0){
			printf("%d",sp);
			break;
		}
		for(i=0;i<n;i+=2){
			if(i == n-1 && a[i] == n){
				swap(&a[i],&a[i-1]);
				sp++;
			}else if(a[i] == i+1){
				swap(&a[i],&a[i+1]);
				sp++;
			}
		}
	}
	return 0;
}