#include <stdio.h>
#include <string.h>
#include <ctype.h>

int main()
{
	int cup[3]={1,0,0};
	int work,ret;
	char order[4];
	while(1){
		ret=scanf("%s", order);
		if(ret==EOF){
			break;
		}
		else if(order[0]=='A'&&order[2]=='B'){
			work=cup[0];
			cup[0]=cup[1];
			cup[1]=work;
		}
		else if(order[0]=='A'&&order[2]=='C'){
			work=cup[0];
			cup[0]=cup[2];
			cup[2]=work;
		}
		else if(order[0]=='B'&&order[2]=='A'){
			work=cup[1];
			cup[1]=cup[0];
			cup[0]=work;
		}
		else if(order[0]=='B'&&order[2]=='C'){
			work=cup[1];
			cup[1]=cup[2];
			cup[2]=work;
		}
		else if(order[0]=='C'&&order[2]=='A'){
			work=cup[2];
			cup[2]=cup[0];
			cup[0]=work;
		}
		else if(order[0]=='C'&&order[2]=='B'){
			work=cup[2];
			cup[2]=cup[1];
			cup[1]=work;
		}
	}
	for(work=0; work<3; work++){
		if(cup[work]==1){
			break;
		}
	}
	if(work==0){
		printf("A\n");
	}
	else if(work==1){
		printf("B\n");
	}
	else{
		printf("C\n");
	}
	return 0;
}