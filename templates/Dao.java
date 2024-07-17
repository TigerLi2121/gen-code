package {{ pkg_name }}.{{ module_name }}.dao;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import {{ pkg_name }}.{{ module_name }}.entity.{{ class_name }}Entity;
import org.apache.ibatis.annotations.Mapper;

/**
 * {{ comment }}
 *
 * @author {{ author }}
 * @date {{ date_time }}
 */
@Mapper
public interface {{ class_name }}Dao extends BaseMapper<{{ class_name }}Entity> {

}