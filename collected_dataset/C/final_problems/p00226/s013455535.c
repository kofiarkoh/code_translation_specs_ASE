#include<string.h>
#include<stdio.h>
int main()
{
	int h,b,i,j;
	char r[5],a[5];
	while(1){
		scanf("%s %s",r,a);
		if(strcmp(r,"0")==0&&strcmp(a,"0")==0){
			break;
		}
		h=0;
		b=0;
		for(i=0;i<4;i++){
			if(r[i]==a[i]){
				h++;
				r[i]='r';
				a[i]='a';
			}
		}
		for(i=0;i<4;i++){
			for(j=0;j<4;j++){
				if(r[i]==a[j]){
					b++;
					r[i]='r';
					a[j]='a';
				}
			}
		}
		printf("%d %d\n",h,b);
	}
	return 0;
}