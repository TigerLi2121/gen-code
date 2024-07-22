package {{ pkg_name }}.{{ module_name}}.entity;

import com.baomidou.mybatisplus.annotation.*;
import lombok.Data;

import java.io.Serializable;
import java.util.Date;

/**
 * {{ comment }}
 *
 * @author {{ author }}
 * @date {{ date_time }}
 */
@Data
@TableName("{{ table_name }}")
public class {{ class_name }}Entity implements Serializable {
    private static final long serialVersionUID = 1L;
    
    {{#each attributes}}
    /**
     * {{ comment }}
     */
    {{#if pk}}
    @TableId(type = IdType.AUTO)
    private {{ attribute_type }} {{ attribute_name_fl }};
    {{else if (eq column_name "created_at")}}
    @TableField(fill = FieldFill.INSERT)
    private {{ attribute_type }} {{ attribute_name_fl }};
    {{else if (eq column_name "updated_at")}}
    @TableField(fill = FieldFill.INSERT_UPDATE)
    private {{ attribute_type }} {{ attribute_name_fl }};
    {{else}}
    private {{ attribute_type }} {{ attribute_name_fl }};
    {{/if}}

    {{/each}}
}