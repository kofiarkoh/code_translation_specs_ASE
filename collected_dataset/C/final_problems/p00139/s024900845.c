#include <stdio.h>

int main(void){
	
	char str[201];
	int i,j,n,cnt1,cnt2,flag;

	scanf("%d",&n);

	for(i=0;i<n;i++){
	
		scanf("%s",str);

		cnt1=0;
		cnt2=0;
		flag=0;

		if(str[0]=='>'){
			
			if(str[1]=='\''){
				for(j=2;str[j]!='\0';j++){
					if(str[j]=='=')
						cnt1++;
					if(str[j+1]=='#')
						break;
				}
				for(j=cnt1+2;str[j]!='\0';j++){
					if(str[j]=='=')
						cnt2++;
				}
				if(cnt1==cnt2 && str[cnt1+cnt2+3]=='~' && str[cnt1+cnt2+4]=='\0'){
					printf("A\n");
					flag=1;
				}
			}		

			else if(str[1]=='^'){
				for(j=2;str[j]!='\0';j++){
					if(j%2 == 0){
						if(str[j]=='Q' && str[j+1]=='='){
							if(str[j+2]=='~' && str[j+3]=='~' && str[j+4]=='\0'){
								printf("B\n");
								flag=1;				
							}
						}
						else
							break;
					}
				}
			
			}
		
		}
		if(flag==0)
			printf("NA\n");
	}
	
	return 0;

}