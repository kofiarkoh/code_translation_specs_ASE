#include <stdio.h>

int search(int s[],int key,int head,int tail){
    if(head>tail)return -1;
    
    int mid=(head+tail)/2;
    
    if(s[mid]==key)return mid;
    else if(s[mid]>key)tail=mid-1;
    else if(s[mid]<key)head=mid+1;
    
    return search(s,key,head,tail);
}
int getLB(int s[],int key,int head,int tail){
    if(s[0]>key)return 0;
    if(s[tail]<key)return tail+1;
    
    int mid=(head+tail)/2;
    
    if(s[mid]<key)head=mid+1;
    else if(s[mid]>key)tail=mid-1;
    else if(s[mid]==key){
        if(s[mid-1]<key)return mid;
        else tail=mid-1;
    }
        
    return getLB(s,key,head,tail);
}
int insert(int s[],int key,int cnt){
    int d;
    if(cnt==0)d=0;
    else d=getLB(s,key,0,cnt-1);
    
    int i;
    for(i=cnt;i>d;i--)s[i]=s[i-1];
    s[i]=key;
    return 1;
}
int delete(int s[],int key,int cnt){
    int d;
    d=search(s,key,0,cnt-1);
    if(d<0)return -1;
    
    int i;
    for(i=d;i<cnt-1;i++)s[i]=s[i+1];
    return 1;
}
int main(void){
    int q;
    scanf("%d\n",&q);
    
    int s[q];
    int cnt=0;
    
    int com,x;
    while(q--){
        scanf("%d %d\n",&com,&x);
        switch(com){
            case 0:
                //insert
                if(search(s,x,0,cnt-1)==-1){
                    if(insert(s,x,cnt)==1)cnt++;
                }
                printf("%d\n",cnt);
                break;
            case 1:
                //find
                if(search(s,x,0,cnt-1)==-1)printf("0\n");
                else printf("1\n");
                break;
            case 2:
                //delete
                if(delete(s,x,cnt)==1)cnt--;
                break;
        }
    }
    
    return 0;
}
