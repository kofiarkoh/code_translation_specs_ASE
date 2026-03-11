#include <stdio.h>
#include <string.h>

int main()
{
	char str[1024];
	int i,j;
	int len;
	int flag=0;
	
	memset(str,'\0',sizeof(str));
	
	while(fgets(str,sizeof(str),stdin)!=NULL){
		flag=0;
		len=strlen(str);
		
		while(1){
			for(i=0;i<len;i++){
				if(str[i]=='t'&&str[i+1]=='h'){
					if(str[i+2]=='e'){
						flag=1;
						break;
					}
					else if(str[i+2]=='i'&&str[i+3]=='s'){
						flag=1;
						break;
					}
					else if(str[i+2]=='a'&&str[i+3]=='t'){
						flag=1;
						break;
					}
				}
			}
			if(flag==0){
				for(i=0;i<len;i++){
					if(str[i]>='a'&&str[i]<='z'){
						if(str[i]=='z'){
							str[i]='a';
						}
						else{
							str[i]++;
						}
					}
				}
			}
			else if(flag=1){
				printf("%s",str);
				break;
			}
		}
		memset(str,'\0',sizeof(str));
	}
	return 0;
}