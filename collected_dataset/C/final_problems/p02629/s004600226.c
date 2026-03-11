#include<stdio.h>
#include <assert.h>
#include <stdio.h>
#include <string.h>

/*
	入力例：
	3
	1 1 3
*/

int main(){
	int i,j;
	long strInput[1];
	for(int i=0;i<1;i++){
	 scanf("%ld", &strInput[i]);
	}

	char strRet[20];
	memset(strRet,0x00,20);
	long tmp=0;
	tmp=strInput[0];
	for(i=0;tmp>0;i++){
			strRet[18-i]='a'+((tmp-1)%26);
//		printf("%d--%c\n",tmp,strRet[15-i]);

		tmp= (int)((tmp - ((tmp-1)%26))/26);
	}
//	printf("end -- %d\n",lastIdx);
	
//	printf("%s\n",strRet[15 - lastIdx +1]);
	char* strRet2;
	for (i=0;i<19;i++){
		if (strRet[i]!=0x00){
			strRet2 = &strRet[i];
			break;
		}
	}
	printf("%s\n",strRet2);
  return 0;
}