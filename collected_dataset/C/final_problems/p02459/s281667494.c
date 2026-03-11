#include <stdio.h>
#include <string.h>

typedef struct{
    char key[20+1];
    int data;
}MAP;

int search(MAP m[],char key[],int head,int tail){
    if(head>tail)return -1;
    
    int mid=(head+tail)/2;
    int r=strcmp(m[mid].key,key);
    
    if(r==0)return mid;
    else if(r>0)tail=mid-1;
    else if(r<0)head=mid+1;
    
    return search(m,key,head,tail);
}
int getLB(MAP m[],char key[],int head,int tail){
    if(tail<0||strcmp(m[0].key,key)>0)return 0;
    if(strcmp(m[tail].key,key)<0)return tail+1;
    
    int mid=(head+tail)/2;
    int r=strcmp(m[mid].key,key);
    
    if(r<0)head=mid+1;
    else if(r>=0){
        if(mid-1<0||strcmp(m[mid-1].key,key)<0)return mid;
        else tail=mid-1;
    }
    
    return getLB(m,key,head,tail);
}
int main(void){
    int q;
    scanf("%d\n",&q);
    
    MAP m[q];
    int cnt=0;
    
    int com,x;
    char key[20+1];
    int d,i;
    while(q--){
        scanf("%d ",&com);
        switch(com){
            case 0:
                //insert
                scanf("%s %d\n",key,&x);
                d=search(m,key,0,cnt-1);
                if(d!=-1){
                    strcpy(m[d].key,key);
                    m[d].data=x;
                }else{
                    d=getLB(m,key,0,cnt-1);
                    for(i=cnt;i<d;i--){
                        strcpy(m[i].key,m[i-1].key);
                        m[i].data=m[i-1].data;
                    }
                    strcpy(m[d].key,key);
                    m[d].data=x;
                    cnt++;
                }
                break;
            case 1:
                //get
                scanf("%s\n",key);
                d=search(m,key,0,cnt-1);
                if(d!=-1)printf("%d\n",m[d].data);
                break;
        }
    }
    
    return 0;
}

