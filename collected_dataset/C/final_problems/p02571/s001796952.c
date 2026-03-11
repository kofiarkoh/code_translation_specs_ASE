#include<stdio.h>
 
int main(void)
{
	int i,min=2000,sum;
	int n=0, m=0, k;
	char s[1001], t[1001];
 
	scanf("%s", &s);
	scanf("%s",&t);
 
for(int i=0;s[i]!='\0';i++)
n++;
for(int i=0;t[i]!='\0';i++)
m++;
 
for(int i=0;i<n-m+1;i++){
 
	sum=0;
	for(int j=0;j<m;j++){
		if(s[i+j]!=t[j]){
			sum++;
		}
	}
	if(sum<2000)
	min=sum;
}
printf("%d",min);
 
	return 0;
}