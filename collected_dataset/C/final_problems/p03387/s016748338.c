#include<stdio.h>
int main(void)
{
int a[3];
scanf("%d %d %d",&a[0],&a[1],&a[2]);

int i,sum=0,max=0;
for(i=0;i<3;++i){
	sum+=a[i];
	if(max<a[i]) max=a[i];
}

int find;
for(i=1;;++i){
	find=3*i;
	if(find>=max*3&&(find-sum)%2==0) break;
}

printf("%d\n",(find-sum)/2);

return 0;
}