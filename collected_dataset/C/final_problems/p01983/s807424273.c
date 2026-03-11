#include<stdio.h>
#include<string.h>
int a,b,c,d;
int cur;
char str[100];
int form(){
    if(str[cur]=='a'){return a;}
    if(str[cur]=='b'){return b;}
    if(str[cur]=='c'){return c;}
    if(str[cur]=='d'){return d;}
    if(str[cur]==']'){
        cur++;
        return form();
    }
    if(str[cur]=='['){
        cur++;
        if(str[cur]=='+'){
            cur++;
            int L=form();
            cur++;
            int R=form();
            return L|R;
        }
        if(str[cur]=='*'){
            cur++;
            int L=form();
            cur++;
            int R=form();
            return L&R;
        }
        if(str[cur]=='^'){
            cur++;
            int L=form();
            cur++;
            int R=form();
            return L^R;
        }
    }
    return 0;
}
int main(void){
    while(1){
        scanf("%s",str);
        if(str[0]=='.') break;
        getchar();
        char abcd[10];
        scanf("%s",abcd); getchar();
        a=abcd[0]-'0'; b=abcd[1]-'0'; c=abcd[2]-'0'; d=abcd[3]-'0';
        cur=0;
        int hash=form();
        int cnt=0;
        for(a=0;a<=9;a++){
            for(b=0;b<=9;b++){
                for(c=0;c<=9;c++){
                    for(d=0;d<=9;d++){
                        cur=0;
                        if(hash==form()){
                            cnt++;
                        }
                    }
                }
            }
        }
        printf("%d %d\n",hash,cnt);
    }
}

