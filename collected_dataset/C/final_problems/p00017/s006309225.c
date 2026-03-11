#include<stdio.h>
#include<string.h>

int main(){
	int i,j,flag=0;
	char s[256];
	fgets(s,sizeof(s),stdin);
	for(i=1;i<26;i++){
		for(j=0;j<strlen(s);j++){
			if(97<=s[j]&&s[j]<=122) {
				if(s[j]+1>122) s[j]=(s[j]+1)%122+96;
				else s[j]=s[j]+1;
			}
		}
		for(j=0;j<strlen(s);j++){
			if(s[j]=='t'){
				if(!strncmp(s+j,"the",3)||!strncmp(s+j,"this",4)||!strncmp(s+j,"that",4)){
					flag=1;
				}
			}
		}
		if(flag) break;
	}
	printf("%s\n",s);
	return 0;
}
	