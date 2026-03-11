#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main(void){
char s[100];
int count,flg=0,counthoji;
char tmp;
char ans[]="KIHBR";
scanf("%s",s);
for(count=0;s[count]!='\0';count++){
if(s[count]=='A'&&s[count+1]=='A'){
    printf("NO");
    return 0;
}
  if(s[count-1]=='K'&&s[count-1]=='A'&&s[count-1]=='I'){
    printf("NO");
    return 0;
}
}
for(;;){
flg=0;
for(count=0;s[count]!='\0';count++){
if(s[count]=='A'){s[count]='0';flg=1;counthoji=count;break;}
}
if(flg==0){break;}
for(count=counthoji;;count++){
s[count]=s[count+1];
if(s[count]=='\0'){break;}
}

}
if(strcmp(s,ans) == 0){printf("YES");}
else{printf("NO");}


return 0;
}