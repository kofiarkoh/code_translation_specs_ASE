#include <stdio.h>

int main(void)
{
    long t,a,b,c,d,i,j,f,o;
    scanf("%ld",&t);
    for(i=0;i<t;i++){
        scanf("%ld%ld%ld%ld",&a,&b,&c,&d);
        f=0;
        o=0;
        if(b>d)f=1;
        if(a%2)o++;
        if(b%2)o++;
        if(d%2)o++;
        if(o==1||o==3){
            if(b>c+1)f=1;
        }
        else{
            if(b>c+2)f=1;
        }
        if(b==d&&a-b-c<=0)f=0;
        if(a<b)f=1;
        puts(f>0?"No":"Yes");
    }
    return 0;
}