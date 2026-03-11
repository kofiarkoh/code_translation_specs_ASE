#include<stdio.h>
#include<string.h>
int main(void)
{
	int a[10+1],b[10+1],c[10+1],ten[10+1],i,j,n,max,cnt,temp;
	char moziretsu[20+1][10+1],mtemp[20+1];
	cnt=0;
	scanf("%d",&n);
	while(n!=0) {
		if(0<cnt) {
			printf("\n");
		}
		for(i=0;i<n;i++) {
			scanf("%s %d %d %d",moziretsu[i],&a[i],&b[i],&c[i]);
			ten[i]=a[i]*3+b[i]*0+c[i]*1;
		}
		for(i=0;i<n-1;i++) {
			for(j=i+1;j<n;j++) {
				if(ten[i]<ten[j]) {
					temp=ten[i];
					ten[i]=ten[j];
					ten[j]=temp;
					strcpy(mtemp,moziretsu[i]);
					strcpy(moziretsu[i],moziretsu[j]);
					strcpy(moziretsu[j],mtemp);
				}
			}
		}
		for(i=0;i<n;i++) {
			printf("%s,%d\n",moziretsu[i],ten[i]);
		}
		cnt++;
	scanf("%d",&n);
	}
	return 0;
}