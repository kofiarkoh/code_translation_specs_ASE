#include <stdio.h>

int main(void){
int many[5]={0};
int hoge,hogera,math=0;
char s[128];

scanf("%d",&hoge);
for(hogera=0;hogera<hoge;hogera++){
scanf("%s",&s);
switch(s[0]){
case 'M':many[0]++;break;
case 'A':many[1]++;break;
case 'R':many[2]++;break;
case 'C':many[3]++;break;
case 'H':many[4]++;break;
}
}

math += (many[0]*many[1]*many[2]);
math += (many[0]*many[1]*many[3]);
math += (many[0]*many[1]*many[4]);
math += (many[0]*many[2]*many[3]);
math += (many[0]*many[2]*many[4]);
math += (many[0]*many[3]*many[4]);
math += (many[1]*many[2]*many[3]);
math += (many[1]*many[2]*many[4]);
math += (many[1]*many[3]*many[4]);
math += (many[2]*many[3]*many[4]);
printf("%d",math);
return 0;
}