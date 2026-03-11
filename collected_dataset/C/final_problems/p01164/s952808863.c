#include <stdio.h>

int main(void) {
	int n;
	char src[20];
	char dest[20];
	char temp;
	int i,j;
	int count,last_count;
	while(1) {
		scanf("%d",&n);
		if(n==0)break;
		scanf("%s",src);
		scanf("%s",dest);
		count=last_count=0;
		for(i=0;i<n && src[i]==dest[i];i++);
		for(j=0;i<n;i++) {
			if(src[i]!=dest[i]) {
				if(j>i)count+=j-i;
				else if(j>0 && i>j)count+=i-j;
				for(j=i+1;j<n;j++) {
					if(src[j]==dest[i])break;
				}
				count+=j-i;
				temp=src[i];
				src[i]=src[j];
				src[j]=temp;
			}
		}
		printf("%d\n",count);
	}
	return 0;
}