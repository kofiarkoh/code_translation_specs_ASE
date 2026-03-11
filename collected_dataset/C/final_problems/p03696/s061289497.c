#include<stdio.h>
#include<string.h>
int main(){
    int no,i,left=0,right=0;
    scanf("%d",&no);
    getchar();
    char s[200];
    gets(s);
    for(i=0;i<no;i++){
        if(s[i]=='(') left++;
        if(s[i]==')'){
            if(left==0) right++;
            else left--;
        }
    }
    for(i=0;i<right;i++) printf("(");
    printf("%s",s);
    for(i=0;i<left;i++) printf(")");
    printf("\n");
    return 0;
}
