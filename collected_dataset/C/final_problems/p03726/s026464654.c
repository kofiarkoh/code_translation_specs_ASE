#include<stdio.h>

int main(void){
	int c=0,n,a,b,pt[131072]={0},pm[131072]={0},pm2[131072]={0},i;
	scanf("%d",&n);
	for(i = 1;i < n;i++){
		scanf("%d%d",&a,&b);
		pt[a]++;pt[b]++;pm[a] = b;pm[b] = a;
	}
	for(i = 1;i <= n;i++){
		if(pt[i] == 1){pm2[(pm[i])]++;}
	}
	for(i = 1;i <= n;i++){
		if(pm2[i] > 0){c++;}
		if(pm2[i] >= 2){printf("First\n");return 0;}
	}
	if(c > (n/2)){printf("First\n");}else{printf("Second\n");}
	return 0;
}