#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#define LEN 20
char pat[LEN+1];
char str[10][LEN+1];
int lp, count;
int check(char *s, int k){
    int i,j;
    char w[LEN+1]; strcpy(w,s);
    for(i=1; i<=25; i++){
        for(j=0; j<k; j++){
            if(w[j] == 'z'+1) return 0;
            if(w[j] == 'a'+i){
                w[j]--;
                break;
            }
        }
    }
    for(j=0; j<k; j++)
        if(w[j] != pat[j]) return 0;
    return 1;
}
void search(char *s, int d){
    int ret;
    ret=check(s,d);
    if(ret==0) return;
    if(s[d]=='\0'){
        if(count<10){
            strcpy(str[count],s);
        }
        else{
            strcpy(str[lp++],s);
            if(lp>=10) lp=5;
        }
        count++;
    }
    else{
        search(s,d+1);
        if(s[d]!='z'){
            s[d]++;
            search(s,d+1);
            s[d]--;
        }
    }
}
int main(void){
    int i,j,k;
    char work[LEN+1];
    while(1){
        scanf("%s",pat);
        if(strcmp(pat,"#")==0) break;
        count=0;
        lp=5;
        strcpy(work,pat);
        search(work,0);
        printf("%d\n",count);
        if(count<=10)
            for(i=0;i<count;i++)
                printf("%s\n",str[i]);
        else{
            for(i=0;i<5;i++)
                printf("%s\n",str[i]);
            for(i=0;i<5;i++){
                printf("%s\n",str[lp++]);
                if(lp>=10) lp=5;
            }
        }
    }
    return 0;
}

