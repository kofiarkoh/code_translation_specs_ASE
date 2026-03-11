#include<stdio.h>
#include<math.h>
#include<string.h>

int main(){
	
	char flag[1000001];
	memset(flag,'0',1000001);
	flag[1000000]='\0';
	
	
	int s;
	int i=0;
	scanf("%d",&s);
	flag[s]=1;
	for(i=2;;i++){
		if(s%2==0){
			s/=2;
			//puts("a");
		}else{
			s=(s*3+1);
			//puts("b");
		}
		
		if(flag[s]=='0'){
			flag[s]=1;
			//puts("c");
		}else{
			printf("%d\n",i);
			//puts("d");
			break;
		}
	}
	return 0;
}