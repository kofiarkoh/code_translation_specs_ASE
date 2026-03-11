#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <inttypes.h>

int main(void){
	int n;
	char s[1000001]={'\0'};
	int i;
	int len;
	int r=0;
	int l=0;
	int j;
	int rflag1=0,rflag2=0;
	int lflag1=0,lflag2=0;
	int lans=0,rans=0;

	scanf("%d",&n);
	for(i=0;i<n;i++){
		scanf("%s",s);
		len=strlen(s);
		for(j=0,l=0,r=0,lflag1=0,rflag1=0;j<len;j++){
			if(s[j]=='('){
				l++;
				lans++;
			}else{
				r++;
				rans++;
			}
			if(j==0){
				if(s[j]=='('){
					lflag1=1;
				}
			}
		}
		if(lflag1==1){
			if(l==r){
				if(s[len-1]==')'){
					lflag1=0;
				}
			}
		}else{
			if(s[len-1]==')'){
				rflag1=1;
			}
		}
		if(lflag1==1){
			lflag2=1;
		}
		if(rflag1==1){
			rflag2=1;
		}
	}
	if(lans==rans){
		if(lflag2==1&&rflag2==1){
			printf("Yes");
			return 0;
		}
	}
	printf("No");

	return 0;
}
