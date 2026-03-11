#include<stdio.h>
#include<string.h>

char a[1005],b[1005];
int main(){
	int i,j,al,bl;
	
	scanf("%s",a);
	scanf("%s",b);
	al=strlen(a);
	bl=strlen(b);
	for(i=0;i<=al-bl;i++){
		for(j=0;j<bl;j++){
			if(b[j]=='_')continue;
			if(a[i+j]!=b[j])break;
		}
		if(j==bl)break;
	}
	if(i==al-bl+1)printf("No\n");
	else printf("Yes\n");
	
	
	return 0;
}
