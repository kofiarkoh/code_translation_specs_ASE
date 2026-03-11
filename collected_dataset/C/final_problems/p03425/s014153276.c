#include <stdio.h>
#include <stdlib.h>
#include <math.h>


#define rep(i,n) for(int i = 0;i<n;i++)
#define getint(n) scanf("%d",&n)

int main(int argc, char const *argv[])
 {
 	int  n = 0;
 	char c = 0;
 	char name[16]="";
 	int names[5] = {};

 	scanf("%d",&n);

 	rep(i, n){
 		scanf("%s",name);
 		switch(*name){
 			case 'M':names[0]++;break;
 			case 'A':names[1]++;break;
 			case 'R':names[2]++;break;
 			case 'C':names[3]++;break;
 			case 'H':names[4]++;break;
 		}
 	}
long long sum = 0;
 	for(int i = 0;i<=2;i++){
 		for(int j = i+1; j<=3;j++ ){
 			for(int k=j+1;k<=4;k++){
 				sum+=names[i]*names[j]*names[k];
 			}
 		}
 	}
 	printf("%lld\n",sum);


 	return 0;
 }