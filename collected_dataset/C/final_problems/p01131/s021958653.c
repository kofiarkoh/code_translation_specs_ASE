#include <stdio.h>
#include <string.h>

int main(void){
    char key[9][6]={".,!? ","abc","def","ghi","jkl","mno","pqrs","tuv","wxyz"};
    char input[1025];
    int mcount[9]={5,3,3,3,3,3,4,3,4};
    int n,count;
    int i,j;
    scanf("%d",&n);
    for(i=0;i<n;i++){
        count=0;
        scanf("%s",input);
        for(j=1;j<=strlen(input);j++){
            if(input[j]!='0')count++;
            if(input[j]=='0' && input[j-1]!='0'){
                printf("%c",key[input[j-1]-'1'][(count%mcount[input[j-1]-'1'])]);
                count=-1;
            }
            if(input[j]=='0' && input[j-1]=='0'){
                printf("");
                count=-1;
            }
            if(input[j]=='\0' && input[j-1]!='0'){
                printf("%c",key[input[j-1]-'1'][(count-1)%(mcount[input[j-1]-'1'])]);
                count=-1;
            }
        }
        printf("\n");
    }
    return 0;
}