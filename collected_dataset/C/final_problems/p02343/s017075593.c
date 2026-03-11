#include <stdio.h>
#include <stdlib.h>

typedef struct _set{
    int *element;
    int eCnt;
    struct _set *next;
}SET;

SET* initSet(int n){
    int i;
    SET *head,*now;
    
    for(i=0;i<n;i++){
        if(i==0){
            head=(SET*)malloc(sizeof(SET));
            now=head;
        }else{
            now->next=(SET*)malloc(sizeof(SET));
            now=now->next;
        }
        now->element=(int*)malloc(sizeof(int));
        now->element[0]=i;
        now->eCnt=1;
    }
    now->next=NULL;
    
    return head;
}

void printSet(SET *head){
    int i;
    SET *now;
    now=head;
    printf("-----printSet-----\n");
    while(now!=NULL){
        for(i=0;i<now->eCnt;i++)printf("%d ",now->element[i]);
        printf("eCnt=%d\n",now->eCnt);
        now=now->next;
    }
    printf("-----end-----\n");
}

void del(SET *head,SET *target){
    SET *p=head;
    if(target==head){
        head=head->next;
//        free(p->element);
//        free(p);
    }else{
        while(p->next!=target)p=p->next;
    
        p->next=target->next;
        free(target->element);
        free(target);
    }
}
void delAll(SET *head){
    SET *now=head;
    while(now!=NULL){
        now=now->next;
        free(head->element);
        free(head);
        head=now;
    }
}

SET* getTarget(SET *head,int target){
    int i;
    SET *now=head;
    
    while(now!=NULL){
        for(i=0;i<now->eCnt;i++){
            if(now->element[i]==target)return now;
        }
        now=now->next;
    }
    return NULL;
}

int unite(SET *head,int x,int y){
    int i,j;
    SET *xp,*yp;
    xp=getTarget(head,x);
    yp=getTarget(head,y);
    if(xp==NULL||yp==NULL||xp==yp)return -1;
    
    xp->element=(int*)realloc(xp->element,sizeof(int)*(xp->eCnt+yp->eCnt));
    for(i=xp->eCnt,j=0;j<yp->eCnt;i++,j++){
        xp->element[i]=yp->element[j];
    }
    xp->eCnt=xp->eCnt+yp->eCnt;
    
    del(head,yp);
    
    return 0;
}

void same(SET *head,int x,int y){
    SET *xp,*yp;
    xp=getTarget(head,x);
    yp=getTarget(head,y);
    if(xp==yp)printf("1\n");
    else printf("0\n");
}
int main(void){
    int i;
    
    int n;
    scanf("%d ",&n);

    SET *head;
    head=initSet(n);
    
    int q;
    scanf("%d\n",&q);
    
    int com,x,y;
    for(i=0;i<q;i++){
        scanf("%d %d %d\n",&com,&x,&y);
        switch(com){
            case 0:
                (void)unite(head,x,y);
                break;
            case 1:
                same(head,x,y);
                break;
        }
//        printSet(head);
    }
    
    delAll(head);
    
    return 0;
}
