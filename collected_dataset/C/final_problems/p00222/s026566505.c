#include <stdio.h>
#define MAX 1000000
int main(void){
        int i,j;
        int a[MAX+1];
        int counter;
        int n;
	counter = 0;
	for(i = 1; i <= MAX; i++){
		a[i] = 0;
	}
	for(i = 2; i <= MAX; i++){
		if(a[i]){continue;}
		for(j = i; j <= MAX; j += i){
			if(j == i){a[i] = 0;}
			else{a[j] = 1;}
		}
	}
	for(i = 2; i <= MAX; i++){
		if(!a[i]){
			counter++;
		}
	}
	while(1){
		scanf("%d",&n);
		if(n == 0){break;}
		i = n;
		while(1){
			for(; a[i]; i--){}
			if(!a[i-8] && !a[i-6] && !a[i-2]){
				printf("%d\n",i);
				break;
			}
			i--;
		}  
	}
        return 0;
}