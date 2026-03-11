#include <stdio.h>
#include <string.h>

int main(void){
	int i=0,j,c,max=0,ans1=100,ans2=100,count=0;
	char s[1000][33]={0};
	char ch;
	
	while(1){
		scanf("%c",&ch);
		if(ch=='\n')break;
		if(ch==' '){
			count++;
			i=0;
		}
		s[count][i]=ch;
		if(max<1+i){
			max=i+1;
			ans2=count;
		}
		if(ch!=' ')i++;
	}
	max=0;
	for(i=0;i<count;i++){
		for(j=i+1,c=0;j<count;j++){
			if(strcmp(s[i],s[j])==0){
				c++;
			}
			if(max<c){
				max=c;
				ans1=i;
			}
		}
	}
	
	printf("%s %s\n",s[ans1],s[ans2]);
	return 0;
}