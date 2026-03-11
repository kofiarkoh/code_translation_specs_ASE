#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>
int main(void){
	char s[51],t[51];
	scanf("%s%s",s,t);
	int i,len1 = strlen(s), len2 = strlen(t),j;
	
	if(len1<len2){
		printf("UNRESTORABLE");
		return 0;
	}
	int k;
	for(i=len1-len2;i>=0;i--){
		if(s[i] == t[0]){
			k = i;
			for(j=1;j<len2;j++){
				if(s[k+1] =='?'){
					s[k+1] = t[j];
					k++;
				}else if(s[k+1] == t[j]){
					k++;
				}else{
					printf("UNRESTORABLE");
					return 0;
				}
			}
			if(j==len2){
				break;
			}
		}
	}
	if(i==-1){
		printf("UNRESTORABLE");
		return 0;
	}
	for(i=0;i<len1;i++) if(s[i] == '?') s[i] = 'a';
	printf("%s",s);
	return 0;
}