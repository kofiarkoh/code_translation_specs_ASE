#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

void inp_str(int*);

int main(void) {

int input1[150];
inp_str(input1);

int X,A;

X=input1[0];
A=input1[1];


//printf("N=%d\n",N);
//printf("X=%d\n",X);
//printf("%d\n",temp);




if(X<A){
	printf("0\n");
} else {
	printf("10\n");

}


    return 0;

}


void inp_str(int instr[]){
char *ptr;
int z,i,count;
char str[20000]={'\0'};
char *ln;

fgets(str, 20000, stdin); // 入力："123456\n"
ln = strchr(str, '\n'); /* 改行文字を検索 */
if (ln != NULL) { /* 改行が読み取られていたかどうか */
   *ln = '\0';    /* 改行文字を終端文字に置き換える */
}
else {            /* 入力ストリーム上に文字が残ってる場合 */
   while (1) {    /* 改行文字が読み取られるまで空読みする */
       z = getchar();
      if (z == '\n' || z == EOF) break;
   }
}
	count=0;
    ptr = strtok(str, " ");
    i = atoi(ptr);
    instr[count]=i;
    count=count+1;
    // 2回目以降
    while(ptr != NULL) {
        // strtok関数により変更されたNULLのポインタが先頭
        ptr = strtok(NULL, " ");
        // ptrがNULLの場合エラーが発生するので対処
        if(ptr != NULL) {
		    i = atoi(ptr);
    instr[count]=i;
    		count=count+1;
        }
    }
}

