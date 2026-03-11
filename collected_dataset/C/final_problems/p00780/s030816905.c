#include<stdio.h>
#include<math.h>

int prime(int a){
	int i,flag=1;
	int a_i=(int)sqrt(a);
	
	for(i=2;i<=a_i;i++){
		if(a%i==0){
			flag=0;
		}
	if(flag==0) break;
	}
	
	return (flag);
}

int main (void){
	
	int i,j,count=0,n;
	
	for(;;){
	scanf("%d",&n);
	
	if(n==0) break;
	
	count=0;
	for(i=2;i<=n;i++){
		for(j=i;j<=n;j++){
			if(prime(i)==1 && prime(j)==1){
				if(i+j==n){
					count++;
				}
			}
		}
	}
	
	
		printf("%d\n",count);
	
}
	
return 0;
}