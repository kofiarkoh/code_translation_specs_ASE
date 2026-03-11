#include<stdio.h>
#include<math.h>
#include <stdlib.h>
#include<string.h>
#include<limits.h>
#include<time.h>

void sosu(int n){
	int i,j,num[n];
	
	for(i=2;i<=n;i++)num[i]=i;
	for(i=2;i<=sqrt(n);i++){
		for(j=2;j*i<=n;j++)num[j*i]=0;
	}
}

int main(){
	int number[10000]={0};
	int ac[10000]={0};
	int i,j,k,count;
	
	for(i=0;scanf("%d,%d",&number[i],&ac[i])!=EOF;i++);
	for(j=i;scanf("%d,%d",&number[j],&ac[j])!=EOF;j++);
	count=j;
	
	for(i=0;i<count;i++){
		for(j=0;j<count;j++){
			if(number[i]<number[j]){
				k=number[i];
				number[i]=number[j];
				number[j]=k;
			}
		}
	}
	
	for(i=0,k=0;i<count;i++){
		for(j=i;j<count;j++){
			if(number[i]==number[j] && i!=j)k++;
		}
		i+=k;
		if(k>=1)printf("%d %d\n",number[i],k+1);
		k=0;
	}
	
return 0;

}