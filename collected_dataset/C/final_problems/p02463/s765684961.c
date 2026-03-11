#include <stdio.h>

int a[200010],b[200010];
int main(){
	int n,m;
	scanf("%d",&n);
	for(int i=0;i<n;i++)scanf("%d",a+i);
	scanf("%d",&m);
	for(int i=0;i<m;i++)scanf("%d",b+i);
	a[n]=b[m]=2e9;
	for(int i=0,j=0;i!=n||j!=m;){
		if(a[i]<b[j])printf("%d\n",a[i++]);
		else if(a[i]>b[j])printf("%d\n",b[j++]);
		else printf("%d\n",a[i]),i++,j++;
	}
}
