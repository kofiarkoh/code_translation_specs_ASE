/*
 * ID: ITP2_1_D
 * Problem: Vector 2
 */

#include <stdio.h>
#include <stdlib.h>

struct _child {
    int value;
    struct _child* child;
};

struct _parent {
    struct _child* child;
    struct _child* last;
};

int main(void)
{
    struct _parent *data;

    int slots, commands;
    scanf("%d %d", &slots, &commands);
    data = malloc(slots * sizeof(struct _parent));
    if (!data) {
        printf("malloc error at _parent\n");
        exit(1);
    }

    int i;
    for (i = 0; i < slots; i++) {
        data[i].child = NULL;
        data[i].last = NULL;
    }

    for (i = 0; i < commands; i++) {
        int slot, command;
        scanf("%d %d", &command, &slot);

        switch (command) {
                int value;
                struct _child *p, *q, *r;
            case 0:
                scanf("%d", &value);
                p = malloc(sizeof(struct _child));
                if (!p) {
                    printf("malloc error at _child\n");
                    exit(1);
                }
                p->value = value;
                p->child = NULL;

                q = data[slot].last;
                if (q) {
                    q->child = p;
                } else {
                    data[slot].child = p;
                }
                data[slot].last = p;
                break;

            case 1:
                q = data[slot].child;
                while (q) {
                    printf("%d", q->value);
                    q = q->child;
                    if (q)
                        putchar(' ');
                }
                printf("\n");
                break;

            case 2:
                q = data[slot].child;
                while (q) {
                    r = q->child;
                    free(q);
                    q = r;
                }
                data[slot].child = NULL;
                data[slot].last = NULL;
                break;
        }
    }

    return 0;
}

