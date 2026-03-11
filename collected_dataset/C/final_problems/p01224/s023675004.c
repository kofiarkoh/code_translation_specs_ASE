#include<stdio.h>

int main(void){
int n[100];
int i,j,s,t,x,z;
int sum=0;


i=0;
do{
scanf("%d",&n[i]);
i++;
}while(n[i-1]!=0);

for(i=0;n[i]!=0;i++){
	s=n[i];
	t=n[i]/100+1;
	z=0;
		for(j=1;j<t;j++){
			if(s%j==0) sum+=j;
		}
		for(j=2;j<100;j++){
			if(s%j==0){
				x=s/j;
				if(s%x==0) sum+=x;
			}
		}
    if(sum==n[i]) printf("perfect number\n");
    else if(sum>n[i]) printf("abundant number\n");
    else if(sum<n[i]) printf("deficient number\n");
    sum=0;
}

return 0;
}