package {{ pkg_name }}.{{ module_name}}.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
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
     */{{#if pk}}
    @TableId(type = IdType.AUTO){{/if}}
    private {{ attribute_type }} {{ attribute_name_fl }};

    {{/each}}
}